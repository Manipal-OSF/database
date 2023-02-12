import type { LayoutServerLoad } from './$types';

export const load = (async ({ cookies }) => {
	const token = cookies.get('token');

	return {
		token: token
	};
}) satisfies LayoutServerLoad;
