import type { Cookies } from '@sveltejs/kit';
import { decodeJwt } from 'jose';

export interface JwtData {
	display_name: string;
	email: string;
	profile_id: string;
	username: string;
	server_id: string;
	private_key: string;
	sub: string;
	iat: number;
	exp: number;
}

export interface AuthDataReturn {
	authorized: boolean;
	data: JwtData | null;
}

export const check_auth = async (cookies: Cookies): Promise<AuthDataReturn> => {
	const auth_key = cookies.get('authorization_key');
	if (!auth_key) return { authorized: false, data: null };
	let jwt: JwtData;
	try {
		jwt = decodeJwt<JwtData>(auth_key);
	} catch {
		return { authorized: false, data: null };
	}
	if (jwt.sub && jwt.username && jwt.email) {
		return {
			authorized: true,
			data: jwt
		};
	}
	return { authorized: false, data: null };
};
