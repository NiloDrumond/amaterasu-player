import { api, type AxiosHelper } from '$lib/axios';
import axios from 'axios';
import type {
  NDAuthenticate,
  NDAlbumListParams,
  NDAlbumListResponse,
  NDAuthenticateParams,
  NDAlbum,
} from './types';

class NDClient {
  private api: AxiosHelper;
  private subsonicCredential: string | null;
  private interceptorId: number | null;

  public constructor(api: AxiosHelper) {
    this.api = api;
    this.subsonicCredential = null;
    this.interceptorId = null;
  }

  public setAuth(data: NDAuthenticate, onUnauthenticated: () => void): void {
    this.api.setToken(data.token);
    this.subsonicCredential = `u=${data.username}&s=${data.subsonicSalt}&t=${data.subsonicToken}`;
    this.interceptorId = this.api.createInterceptor(undefined, (error) => {
      if (error.response?.status === 401) {
        this.signOut();
        onUnauthenticated();
      }
    });
  }

  public async authenticate(
    params: NDAuthenticateParams,
  ): Promise<NDAuthenticate> {
    try {
      const response = await this.api.post<NDAuthenticate>({
        url: '/auth/login',
        body: params,
      });
      const data = response.data;
      return data;
    } catch (e) {
      if (axios.isAxiosError(e)) {
        if (e?.response?.status === 401) {
          throw new Error('Wrong username/password');
        }
      }
      throw new Error('Unexpected Error');
    }
  }

  public signOut(): void {
    this.api.clearToken();
    this.subsonicCredential = null;
    if (this.interceptorId) {
      this.api.removeInterceptor(this.interceptorId);
    }
  }

  public async getAlbumList(
    args: NDAlbumListParams,
  ): Promise<NDAlbumListResponse> {
    const response = await this.api.get<NDAlbumListResponse>({
      url: '/api/album',
      config: { params: args },
    });
    return response.data;
  }

  public async getAlbum(args: { albumId: string }): Promise<NDAlbum> {
    const response = await this.api.get<NDAlbum>({
      url: `/api/album/${args.albumId}`,
    });
    return response.data;
  }

  public getCoverArtUrl(args: {
    baseUrl?: string;
    coverArtId: string;
    size?: number;
  }): string {
    const size = args.size ? args.size : 250;
    const baseUrl = this.api.prepareUrl('/rest/getCoverArt');
    return (
      baseUrl +
      `?${this.subsonicCredential}` +
      `&id=${args.coverArtId}` +
      `&size=${size}` +
      `&v=1.13.0` +
      `&f=json` +
      `&c=AmaterasuPlayer`
    );
  }
}

export const ndClient = new NDClient(api);
