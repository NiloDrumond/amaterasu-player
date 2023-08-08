import axios, {
  type AxiosInstance,
  type AxiosRequestConfig,
  type AxiosResponse,
} from 'axios';
import type { GetParams, PostParams } from './types';
import { PUBLIC_API_URL } from '$env/static/public';

export class AxiosHelper {
  private token: string | null;
  private instance: AxiosInstance;

  public constructor() {
    this.token = null;
    this.instance = axios.create();
  }

  public setToken(token: string) {
    this.token = token;
  }

  public clearToken() {
    this.token = null;
  }

  private prepareUrl(url: string) {
    if (url.includes('http')) return url;
    return PUBLIC_API_URL + url;
  }

  private prepareConfig(params?: AxiosRequestConfig): AxiosRequestConfig {
    const header =
      this.token !== null
        ? { 'x-nd-authorization': `Bearer ${this.token}` }
        : undefined;
    const config = {
      ...params,
      headers: { ...header, ...(params ? params.headers : {}) },
    };
    return config;
  }

  public async get<T = unknown, R = AxiosResponse<T, unknown>>(
    params: GetParams,
  ): Promise<R> {
    const url = this.prepareUrl(params.url);
    return this.instance.get(url, this.prepareConfig(params.config));
  }

  public async post<T = unknown, R = AxiosResponse<T, unknown>>(
    params: PostParams,
  ): Promise<R> {
    const url = this.prepareUrl(params.url);

    return this.instance.post(url, params.body, this.prepareConfig());
  }
}

export const api = new AxiosHelper();
