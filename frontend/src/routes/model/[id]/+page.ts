import type { FullModelWithRelationsIds } from '$lib/helpers/typings';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
export const load: PageLoad = async ({ fetch, params, url }) => {
	const res = await fetch(`/api/v1/model/${params.id}`);
	const own = Boolean(url.searchParams.get('own') ?? false);
	if (!res.ok) {
		throw error(res.status, await res.text());
	}
	const model: FullModelWithRelationsIds = await res.json();
	return { model: model, own };
};
