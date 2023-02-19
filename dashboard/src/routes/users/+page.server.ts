import type { Actions, PageServerLoad } from './$types';
import { PUBLIC_SERVER_URL } from '$env/static/public';
import { error, fail } from '@sveltejs/kit';

export const ssr = true;

export interface UserModel {
	registrationNumber: number;
	name: string;
	title: string | undefined;
	phoneNumber: number;
	email: string;
	designation: string | undefined;
	department: string | undefined;
	year: number;
	remarks: string | undefined;
	strikes: number;
}

export const load: PageServerLoad = async ({ cookies }) => {
	const token = cookies.get('token');

	if (typeof token === 'undefined') {
		throw error(401, 'Not logged in / Session expired');
	}

	const resp = await fetch(
		`${PUBLIC_SERVER_URL ?? 'http://127.0.0.1:8000'}/api/v1/dashboard/users`,
		{
			method: 'GET',
			headers: { Authorization: `Bearer ${token}` },
		}
	);

	const data: Array<UserModel> = await resp.json();

	return {
		users: data.map((val) => {
			return val;
		}),
	} satisfies { users: UserModel[] };
};

const checkAndConvertUndefined = (data: string): string | undefined => {
	if (data.trim() === '') {
		return undefined;
	}
	return data;
};

export const actions: Actions = {
	default: async (event) => {
		const data = await event.request.formData();
		const method = String(data.get('method'));

		const body: UserModel = {
			registrationNumber: Number(data.get('registrationNumber')),
			name: String(data.get('name')),
			title: checkAndConvertUndefined(String(data.get('title'))),
			phoneNumber: Number(data.get('phoneNumber')),
			email: String(data.get('email')),
			designation: checkAndConvertUndefined(String(data.get('designation'))),
			department: checkAndConvertUndefined(String(data.get('department'))),
			year: Number(data.get('year')),
			remarks: checkAndConvertUndefined(String(data.get('remarks'))),
			strikes: Number(data.get('strikes')),
		};

		try {
			const resp = await event.fetch(
				`${PUBLIC_SERVER_URL ?? 'http://127.0.0.1:8000'}/api/v1/dashboard/users`,
				{
					method: method,
					body: JSON.stringify(body),
					headers: {
						'Content-Type': 'application/json',
						Authorization: `Bearer ${event.cookies.get('token')}`,
					},
				}
			);

			if (resp.status === 200) {
				return { success: true };
			} else {
				const json = await resp.json();
				return fail(resp.status, { message: json.message });
			}
		} catch (err) {
			return fail(500, { message: 'Server down' });
		}
	},
};
