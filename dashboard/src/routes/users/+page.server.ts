import type { Actions, PageServerLoad } from './$types';

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
	const resp = await fetch('http://127.0.0.1:8000/api/v1/dashboard/users', {
		method: 'GET',
		headers: { Authorization: `Bearer ${cookies.get('token')}` },
	});

	const data: Array<{
		RegistrationNumber: number;
		Name: string;
		Title: string | undefined;
		PhoneNumber: number;
		Email: string;
		Designation: string | undefined;
		Department: string | undefined;
		Year: number;
		Remarks: string | undefined;
		Strikes: number;
	}> = await resp.json();

	return {
		users: data.map((val) => {
			return {
				registrationNumber: val['RegistrationNumber'],
				name: val['Name'],
				title: val['Title'],
				phoneNumber: val['PhoneNumber'],
				email: val['Email'],
				designation: val['Designation'],
				department: val['Department'],
				year: val['Year'],
				remarks: val['Remarks'],
				strikes: val['Strikes'],
			} satisfies UserModel;
		}),
	} satisfies { users: UserModel[] };
};

export const actions: Actions = {
	default: async (event) => {
		const data = await event.request.formData();

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

		const resp = await event.fetch('http://127.0.0.1:8000/api/v1/dashboard/users', {
			method: 'PATCH',
			body: JSON.stringify(body),
			headers: { 'Content-Type': 'application/json' },
		});
		const json = await resp.json();
		console.log(JSON.stringify(json));

		if (resp.status === 200) {
			return { success: true };
		}
		return { success: false };
	},
};
