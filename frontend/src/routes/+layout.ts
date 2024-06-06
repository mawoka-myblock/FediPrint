import { user } from '$lib/stores';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = ({ data }) => {
	user.set(data.user);
};
