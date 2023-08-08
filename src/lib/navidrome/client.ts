import { PUBLIC_AUTH_URL } from '$env/static/public';
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

  public async authenticate(params: NDAuthenticateParams) {
    try {
      const response = await this.api.post<NDAuthenticate>({
        url: `${PUBLIC_AUTH_URL}/login`,
        body: params,
      });
      const data = response.data;
      this.api.setToken(data.token);
      this.subsonicCredential = `u=${data.username}&s=${data.subsonicSalt}&t=${data.subsonicToken}`;
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
      url: '/album',
      config: { params: params },
    });
    return response.data;
  }
}

export const ndClient = new NDClient(api);
