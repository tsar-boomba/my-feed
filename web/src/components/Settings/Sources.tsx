import useSWR from 'swr';
import { apiFetcher, apiUrl } from '../../api';
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
	TextInput,
} from '@mantine/core';
import { useForm } from 'react-hook-form';
import { TbPlus } from 'react-icons/tb';
import { useState } from 'react';
import { toLocaleDateString, toLocaleTimeString } from '../../utils/date';

export const Sources = ({ auth }: { auth: string }) => {
	const {
		data: sources,
		error,
		mutate,
	} = useSWR<Source[]>('/sources', apiFetcher);
	const [creatingSource, setCreatingSource] = useState(false);
	const { register, handleSubmit } = useForm<{ name: string; url: string }>({
		defaultValues: {
			name: '',
			url: '',
		},
	});

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
		<Stack align='center'>
			<Stack
				component='form'
				onSubmit={handleSubmit(async ({ name, url }) => {
					const body: Source = {
						name,
						url,

						id: 0,
						favorite: false,
						last_poll: null,
						ttl: null,
						last_pub: new Date().toISOString(),
						created_at: new Date().toISOString(),
						updated_at: new Date().toISOString(),
					};
					setCreatingSource(true);

					try {
						const res = await fetch(apiUrl('/sources'), {
							method: 'POST',
							body: JSON.stringify(body, null, 4),
							headers: {
								['x-auth']: auth,
								['content-type']: 'application/json',
							},
						});

						if (res.ok) {
							console.log('Created source:', await res.json());
							await mutate();
						} else {
							console.error('Failed to create source:', await res.text());
						}
					} catch (e) {
						console.error(e);
					} finally {
						setCreatingSource(false);
					}
				})}
			>
				<TextInput {...register('name')} label='Name' />
				<TextInput {...register('url')} label='URL' />
				<Button leftSection={<TbPlus />} type='submit' loading={creatingSource}>
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
							{source.last_poll
								? `${toLocaleDateString(source.last_poll)} ${toLocaleTimeString(source.last_poll)}`
								: 'never'}
						</Text>
						<Text>
							Last Published:{' '}
							{source.last_pub
								? `${toLocaleDateString(source.last_pub)} ${toLocaleTimeString(source.last_pub)}`
								: 'never'}
						</Text>
					</Paper>
				))}
			</Stack>
		</Stack>
	);
};
