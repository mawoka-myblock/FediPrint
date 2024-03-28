import type { PageServerLoad } from './$types';
import type { AuthDataReturn } from '$lib/helpers/auth';
import { check_auth } from '$lib/helpers/auth';

export const load: PageServerLoad = async ({ cookies }): Promise<{ user: AuthDataReturn }> => {
	return { user: await check_auth(cookies) };
};
