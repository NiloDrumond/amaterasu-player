import type { AxiosRequestConfig } from 'axios';

export type GetParams = {
  url: string;
  config?: AxiosRequestConfig | undefined;
};

export type PostParams = {
  url: string;
  body: unknown;
  config?: AxiosRequestConfig | undefined;
};
