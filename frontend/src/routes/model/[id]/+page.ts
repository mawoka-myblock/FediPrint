import type { FullModelWithRelationsIds } from '$lib/helpers/typings';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
export const load: PageLoad = async ({ fetch, params }) => {
	const res = await fetch(`/api/v1/model?id=${params.id}`);
    if (!res.ok) {
        throw error(res.status, await res.text())
    }
    const model: FullModelWithRelationsIds = await res.json()
    return {model: model}
};
