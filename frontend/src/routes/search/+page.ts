import { error } from '@sveltejs/kit';
import type { PageLoad, Actions } from './$types';
export interface SearchResult {
	hits: Hit[];
	offset: any;
	limit: any;
	estimated_total_hits: any;
	page: number;
	hits_per_page: number;
	total_hits: number;
	total_pages: number;
	processing_time_ms: number;
}

export interface Hit {
	result: Result;
	formatted_result: any;
	ranking_score: any;
}

export interface Result {
	id: string;
	title: string;
	content: string;
	summary: string;
	created_at: string;
	updated_at: string;
	tags: string[];
	profile_id: string;
	record_type: string;
}

export const load: PageLoad = async ({ url, fetch }) => {
	const query = url.searchParams.get('q') ?? '';
	let page = parseInt(url.searchParams.get('p') ?? '1');
	if (isNaN(page)) page = 1;
	if (query.length < 3) {
		return { results: null, query, page };
	}
	const res = await fetch(`/api/v1/search/model?q=${query}&page=${page}`);
	if (!res.ok) throw error(500, 'Search failed');
	const results: SearchResult = await res.json();
	return {
		results,
		query,
		page
	};
};
