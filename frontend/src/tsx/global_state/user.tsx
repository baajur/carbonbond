import { API_FETCHER, unwrap } from '../../ts/api/api';
import * as React from 'react';
import { toast } from 'react-toastify';
import { createContainer } from 'unstated-next';
const { useState } = React;

type UserStateType = {
	login: false, fetching: boolean
} | {
	login: true,
	id: number,
	user_name: string,
	invitation_credit: number,
	energy: number
};

interface LoginData {
	id: number,
	user_name: string,
	invitation_credit: number,
	energy: number
}

function useUserState(): { user_state: UserStateType, setLogin: Function, setLogout: Function, getLoginState: Function } {
	const [user_state, setUserState] = useState<UserStateType>({ login: false, fetching: true });

	async function getLoginState(): Promise<void> {
		try {
			const user = unwrap(await API_FETCHER.queryMe());
			if (user) {
				setUserState({
					login: true,
					user_name: user.user_name,
					id: user.id,
					invitation_credit: user.invitation_credit,
					energy: user.energy,
				});
			} else {
				setUserState({ login: false, fetching: false });
			}
		} catch (err) {
			toast.error(err);
		}
		return;
	}

	React.useEffect(() => {
		getLoginState();
	}, []);

	function setLogin(data: LoginData): void {
		setUserState({
			login: true,
			id: data.id,
			user_name: data.user_name,
			invitation_credit: data.invitation_credit,
			energy: data.energy,
		});
	}
	function setLogout(): void {
		setUserState({ login: false, fetching: false });
	}
	return { user_state, setLogin, setLogout, getLoginState };
}

export const UserState = createContainer(useUserState);