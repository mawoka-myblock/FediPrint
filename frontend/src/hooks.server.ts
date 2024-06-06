import type { Handle, ResolveOptions } from '@sveltejs/kit';
import jws from 'jws';
import type { Claims } from './app';

const uno_data: ResolveOptions = {
	transformPageChunk: ({ html }) =>
		html.replace('%unocss-svelte-scoped.global%', 'unocss_svelte_scoped_global_styles')
};

export const handle: Handle = async ({ event, resolve }) => {
	const access_token = event.cookies.get('authorization_key');
	if (!access_token) {
		event.locals.user = null;
		return resolve(event, uno_data);
	}
	const jwt = jws.decode(access_token)
	const user = jwt?.payload;
	delete user["private_key"]
	event.locals.user = user;
	return resolve(event, uno_data);

}
