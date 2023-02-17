import type { Actions, PageServerLoad } from './$types';
import { SERVER_URL } from '$env/static/private';
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

	const resp = await fetch(`${SERVER_URL ?? 'http://127.0.0.1:8000'}/api/v1/dashboard/users`, {
		method: 'GET',
		headers: { Authorization: `Bearer ${token}` },
	});

	const data: Array<UserModel> = await resp.json();

	return {
		users: data.map((val) => {
			return val;
		}),
	} satisfies { users: UserModel[] };
};

export const actions: Actions = {
	default: async (event) => {
		const data = await event.request.formData();
		const method = String(data.get('method'));

		const body: UserModel = {
			registrationNumber: Number(data.get('registrationNumber')),
			name: String(data.get('name')),
			title: String(data.get('title')),
			phoneNumber: Number(data.get('phoneNumber')),
			email: String(data.get('email')),
			designation: String(data.get('designation')),
			department: String(data.get('department')),
			year: Number(data.get('year')),
			remarks: String(data.get('remarks')),
			strikes: Number(data.get('strikes')),
		};

		try {
			const resp = await event.fetch(
				`${SERVER_URL ?? 'http://127.0.0.1:8000'}/api/v1/dashboard/users`,
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
			}
			return { success: false };
		} catch (err) {
			return fail(500, { message: 'Server down' });
		}
	},
};