import { z } from 'zod';

export const signInSchema = z.object({
  username: z.string().min(1, 'Required'),
  password: z.string().min(1, 'Required'),
});

export type SignInSchema = z.infer<typeof signInSchema>;

export type SignInErrors = z.inferFlattenedErrors<
  typeof signInSchema
>['fieldErrors'] & { message?: string };
