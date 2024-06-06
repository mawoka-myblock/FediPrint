// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			user: Claims | null;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
export interface Claims {
	sub: string;
	email: string;
	username: string;
	display_name: string;
	profile_id: string;
	server_id: string;
	exp: number;
	iat: number;
}
