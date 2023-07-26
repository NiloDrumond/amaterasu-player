import type { Actions } from './$types';
import { AUTH_URL } from '$env/static/private';
import axios from 'axios';
import { ActionFailure, fail } from '@sveltejs/kit';
import type { NDAuthenticate } from '$lib/navidrome/types';
import { signInSchema, type SignInError, type SignInSuccess } from './data';

export const actions: Actions = {
  default: async ({
    request,
  }): Promise<ActionFailure<SignInError> | SignInSuccess> => {
    const formData = Object.fromEntries(await request.formData());
    const signInData = signInSchema.safeParse(formData);
    if (signInData.success) {
      try {
        const response = await axios.post<NDAuthenticate>(
          `${AUTH_URL}/login`,
          signInData.data,
        );
        return { success: true, user: response.data };
      } catch (e) {
        if (axios.isAxiosError(e)) {
          if (e?.response?.status === 401) {
            return fail(400, {
              success: false,
              message: 'Wrong username/password',
            });
          }
        }
        return fail(500, { success: false, message: 'Unexpected error' });
      }
    }
    return fail(400, {
      success: false,
      errors: signInData.error.formErrors.fieldErrors,
    });
  },
};
