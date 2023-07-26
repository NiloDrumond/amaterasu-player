import type { NDAuthenticate } from '$lib/navidrome/types';
import { z } from 'zod';

export const signInSchema = z.object({
  username: z.string().min(1, 'Required'),
  password: z.string().min(1, 'Required'),
});

export type SignInSchema = z.infer<typeof signInSchema>;

export type SignInError = {
  success: false;
  message?: string;
  errors?: z.inferFlattenedErrors<typeof signInSchema>['fieldErrors'];
};

export type SignInSuccess = {
  success: true;
  user: NDAuthenticate;
};

export type SignInOutput = SignInSuccess | SignInError;
