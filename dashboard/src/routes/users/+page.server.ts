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
		headers: { Authorization: `Bearer ${cookies.get('token')}` }
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

	console.log(data);

	return {
		users: data.map((e) => {
			return {
				registrationNumber: e['RegistrationNumber'],
				name: e['Name'],
				title: e['Title'],
				phoneNumber: e['PhoneNumber'],
				email: e['Email'],
				designation: e['Designation'],
				department: e['Department'],
				year: e['Year'],
				remarks: e['Remarks'],
				strikes: e['Strikes']
			} satisfies UserModel;
		})
	} satisfies { users: UserModel[] };
};

export const actions: Actions = {
	default: async (event) => {
		const data = await event.request.formData();
		const key = data.get('apikey');

		console.log('elo');
	}
};
