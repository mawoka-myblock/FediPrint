import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = ({ url, params }) => {
	return redirect(308, `/api/v1/statuses/${params.id}`);
};
