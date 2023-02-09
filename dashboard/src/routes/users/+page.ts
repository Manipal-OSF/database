import type { PageLoad } from "./$types";

export const ssr = true;

export interface UserModel {
    registrationNumber: number;
}

export const load: PageLoad = async () => {
    return {registrationNumber: 2} satisfies UserModel;
};
