import { Button, Center, Group, Loader, Stack, TextInput } from '@mantine/core';
import { useTags } from '../../utils/tags';
import { Tag } from '../Tag';
import { TbPlus } from 'react-icons/tb';
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { Tag as TagType } from '../../types/tag';
import { apiUrl } from '../../api';

export const Tags = ({ auth }: { auth: string }) => {
	const { tags, error, mutate } = useTags();
	const [creatingSource, setCreatingSource] = useState(false);
	const { register, handleSubmit } = useForm<{
		name: string;
		background_color: string;
		text_color: string;
		border_color: string;
	}>({
		defaultValues: {
			name: '',
			background_color: '',
			text_color: '',
			border_color: '',
		},
	});

	if (error) {
		return error.toString();
	}

	if (!tags) {
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
				onSubmit={handleSubmit(
					async ({ name, background_color, border_color, text_color }) => {
						const body: TagType = {
							name,
							background_color,
							border_color,
							text_color,

							created_at: new Date().toISOString(),
							updated_at: new Date().toISOString(),
						};
						setCreatingSource(true);

						try {
							const res = await fetch(apiUrl('/tags'), {
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
					},
				)}
			>
				<TextInput {...register('name')} label='Name' withAsterisk />
				<TextInput
					{...register('background_color')}
					label='Background Color'
					withAsterisk
				/>
				<Button leftSection={<TbPlus />} type='submit' loading={creatingSource}>
					Add Tag
				</Button>
			</Stack>
			<Group px={8}>
				{Array.from(tags.values()).map((tag) => (
					<Tag tag={tag} />
				))}
			</Group>
		</Stack>
	);
};
