import type { PageServerLoad } from './$types';
import { check_auth } from '$lib/helpers/auth';
import { redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ cookies }) => {
	const d = await check_auth(cookies);
	if (d.authorized) {
		redirect(307, '/home');
	}
};
