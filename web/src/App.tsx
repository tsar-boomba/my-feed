import '@mantine/core/styles.css';
import { MantineProvider } from '@mantine/core';
import { theme } from './theme';
import { Outlet } from 'react-router';
import { Layout } from './components/Layout';

export default function App() {
	return (
		<MantineProvider theme={theme}>
			<Layout>
				<Outlet />
			</Layout>
		</MantineProvider>
	);
}
