import { Button, PasswordInput, Stack, Tabs } from '@mantine/core';
import { useLocalStorage } from '@mantine/hooks';
import { useState } from 'react';
import { apiUrl } from '../api';
import { Sources } from '../components/Settings/Sources';
import { Tags } from '../components/Settings/Tags';
import { useAuth } from '../utils/useAuth';

export const Settings = () => {
	const [tab, setTab] = useLocalStorage({
		key: 'settingsTab',
		defaultValue: 'sources'
	})
	const [auth, setAuth] = useAuth();
	const [inputAuth, setInputAuth] = useState('');
	const [checkingAuth, setCheckingAuth] = useState(false);

	if (!auth) {
		return (
			<Stack align='center'>
				<PasswordInput
					value={inputAuth}
					disabled={checkingAuth}
					onChange={(e) => setInputAuth(e.target.value)}
					w={200}
					label='Auth'
				/>
				<Button
					loading={checkingAuth}
					onClick={async () => {
						if (checkingAuth) return;
						setCheckingAuth(true);

						try {
							const res = await fetch(apiUrl('/login'), {
								method: 'POST',
								headers: { ['x-auth']: inputAuth },
							});
							if (res.ok) {
								setAuth(inputAuth);
							}
						} catch (e) {
							console.error(e);
						} finally {
							setCheckingAuth(false);
						}
					}}
				>
					Submit
				</Button>
			</Stack>
		);
	}

	return <Tabs value={tab}>
		<Tabs.List>
			<Tabs.Tab value='sources' onClick={() => setTab('sources')}>Sources</Tabs.Tab>
			<Tabs.Tab value='tags' onClick={() => setTab('tags')}>Tags</Tabs.Tab>
		</Tabs.List>

		<Tabs.Panel value='sources'>
			<Sources auth={auth} />
		</Tabs.Panel>

		<Tabs.Panel value='tags'>
			<Tags auth={auth} />
		</Tabs.Panel>
	</Tabs>;
};
