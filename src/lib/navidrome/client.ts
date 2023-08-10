import { api, type AxiosHelper } from '$lib/axios';
import axios from 'axios';
import type {
  NDAuthenticate,
  NDAlbumListParams,
  NDAlbumListResponse,
  NDAuthenticateParams,
} from './types';

class NDClient {
  private api: AxiosHelper;
  private subsonicCredential: string | null;

  public constructor(api: AxiosHelper) {
    this.api = api;
    this.subsonicCredential = null;
  }

  public setAuth(data: NDAuthenticate) {
    this.api.setToken(data.token);
    this.subsonicCredential = `u=${data.username}&s=${data.subsonicSalt}&t=${data.subsonicToken}`;
  }

  public async authenticate(params: NDAuthenticateParams) {
    try {
      const response = await this.api.post<NDAuthenticate>({
        url: '/auth/login',
        body: params,
      });
      const data = response.data;
      this.setAuth(data);
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

  public signOut() {
    this.api.clearToken();
    this.subsonicCredential = null;
  }

  public async getAlbumList(params: NDAlbumListParams) {
    const response = await this.api.get<NDAlbumListResponse>({
      url: '/api/album',
      config: { params: params },
    });
    return response.data;
  }

  public getCoverArtUrl(args: {
    baseUrl?: string;
    coverArtId: string;
    size?: number;
  }) {
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
