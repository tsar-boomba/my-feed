import { useLocalStorage } from '@mantine/hooks';
import { Dispatch, SetStateAction } from 'react';

export const useAuth = (): [
	string | undefined,
	Dispatch<SetStateAction<string | undefined>>,
	() => void,
] => useLocalStorage<string | undefined>({ key: 'auth', sync: true });
