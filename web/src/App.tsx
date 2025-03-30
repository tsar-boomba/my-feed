import '@mantine/core/styles.css';
import '@mantine/dates/styles.css';
import { MantineProvider } from '@mantine/core';
import { ModalsProvider } from '@mantine/modals';
import { theme } from './theme';
import { Outlet } from 'react-router';
import { Layout } from './components/Layout';

export default function App() {
	return (
		<MantineProvider theme={theme}>
			<ModalsProvider>
				<Layout>
					<Outlet />
				</Layout>
			</ModalsProvider>
		</MantineProvider>
	);
}
