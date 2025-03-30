import useSWR from 'swr';
import { apiFetcher } from '../../api';
import { Source } from '../../types/source';
import {
	Anchor,
	Button,
	Center,
	Divider,
	Loader,
	Paper,
	Stack,
	Text,
} from '@mantine/core';
import { TbPlus } from 'react-icons/tb';
import { toLocaleDateString, toLocaleTimeString } from '../../utils/date';
import { openModal } from '@mantine/modals';
import { AddSourceModal } from './AddSourceModal';

export const Sources = ({ auth }: { auth: string }) => {
	const { data: sources, error } = useSWR<Source[]>('/sources', apiFetcher);

	if (error) {
		return error.toString();
	}

	if (!sources) {
		return (
			<Center>
				<Loader size='xl' />
			</Center>
		);
	}

	return (
		<Stack align='center' py='md'>
			<Stack>
				<Button
					leftSection={<TbPlus />}
					onClick={() =>
						openModal({
							title: 'Add Source',
							children: <AddSourceModal auth={auth} />,
						})
					}
				>
					Add Source
				</Button>
			</Stack>
			<Stack>
				{sources.map((source) => (
					<Paper key={source.id} withBorder shadow='sm' p='md'>
						<Text fw={700}>{source.name}</Text>
						<Anchor c='dimmed' href={source.url} target='_blank'>
							{source.url}
						</Anchor>
						<Divider my='xs' />
						<Text>
							Last Polled:{' '}
							{source.lastPoll
								? `${toLocaleDateString(source.lastPoll)} ${toLocaleTimeString(source.lastPoll)}`
								: 'never'}
						</Text>
						<Text>
							Last Published:{' '}
							{source.lastPub
								? `${toLocaleDateString(source.lastPub)} ${toLocaleTimeString(source.lastPub)}`
								: 'never'}
						</Text>
					</Paper>
				))}
			</Stack>
		</Stack>
	);
};
