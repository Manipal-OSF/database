import { fail, type Actions } from '@sveltejs/kit';

export const actions: Actions = {
	default: async (event) => {
		const data = await event.request.formData();
		const key = data.get('apikey');

		let json;
		try {
			const resp = await event.fetch('http://127.0.0.1:8000/api/v1/dashboard/login', {
				method: 'POST',
				body: JSON.stringify({ api_key: key }),
				headers: { 'Content-Type': 'application/json' },
			});
			json = await resp.json();
		} catch (err) {
			json = undefined;
		}

		if (json['access_token']) {
			event.cookies.set('token', json['access_token'], { maxAge: 60 * 60 });
			return { success: true };
		} else {
			return fail(500, { message: 'Incorrect key!' });
		}
	},
};
