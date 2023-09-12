import { api, type AxiosHelper } from '$lib/axios';
import axios from 'axios';
import type {
  NDAuthenticate,
  NDAlbumListParams,
  NDAlbumListResponse,
  NDAuthenticateParams,
  NDAlbum,
  NDSong,
  NDSongListParams,
  NDSubsonicScanStatusResponse,
  NDPlaylistListParams,
  NDPlaylist,
  NDPlaylistSong,
  NDPlaylistSongListParams,
} from './types';

const COMMON_SUBSONIC_PARAMS = `&v=1.13.0` + `&f=json` + `&c=AmaterasuPlayer`;

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

  public async getPlaylist(args: { playlistId: string }) {
    const response = await this.api.get<NDPlaylist>({
      url: `/api/playlist/${args.playlistId}`,
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
      COMMON_SUBSONIC_PARAMS
    );
  }

  public getSongStreamUrl(args: { songId: string }) {
    const baseUrl = this.api.prepareUrl('/rest/stream');
    return (
      baseUrl +
      `?${this.subsonicCredential}` +
      `&id=${args.songId}` +
      COMMON_SUBSONIC_PARAMS
    );
  }

  public async getSongList(params: NDSongListParams) {
    const response = await this.api.get<NDSong[]>({
      url: '/api/song',
      config: {
        params,
      },
    });

    return response.data;
  }

  public async getScanStatus() {
    const baseUrl = this.api.prepareUrl('/rest/getScanStatus');
    const url =
      baseUrl + `?${this.subsonicCredential}` + COMMON_SUBSONIC_PARAMS;
    const response = await this.api.get<NDSubsonicScanStatusResponse>({ url });
    return response.data;
  }

  public async getPlaylistList(params: NDPlaylistListParams) {
    const response = await this.api.get<NDPlaylist[]>({
      url: '/api/playlist',
      config: {
        params,
      },
    });

    return response.data;
  }

  public async getPlaylistSongList(args: NDPlaylistSongListParams) {
    const { playlist_id, ...params } = args;
    const response = await this.api.get<NDPlaylistSong[]>({
      url: `/api/playlist/${playlist_id}/tracks`,
      config: {
        params,
      },
    });

    return response.data;
  }
}

export const ndClient = new NDClient(api);
