import { KeyedMutator } from 'swr';
import { Tag as TagType } from '../../types/tag';
import { useForm } from 'react-hook-form';
import { Button, Stack, TextInput } from '@mantine/core';
import { useState } from 'react';
import { apiUrl } from '../../api';
import { Tag } from '../Tag';

export const EditTagModal = ({
	tag,
	mutate,
	auth,
}: {
	tag: TagType;
	mutate: KeyedMutator<any>;
	auth: string;
}) => {
	const { register, handleSubmit, watch } = useForm<
		Omit<TagType, 'name' | 'created_at' | 'updated_at'>
	>({
		defaultValues: {
			...tag,
		},
	});
	const values = watch();
	const [updating, setUpdating] = useState(false);

	return (
		<Stack
			component='form'
			onSubmit={handleSubmit(
				async ({ background_color, border_color, text_color }) => {
					if (!background_color || !border_color || !text_color) return;
					setUpdating(true);
					const body: TagType = {
						background_color,
						border_color,
						text_color,
						name: tag.name,

						created_at: '',
						updated_at: '',
					};

					try {
						const res = await fetch(apiUrl('/tags'), {
							method: 'PUT',
							headers: {
								['x-auth']: auth,
								['content-type']: 'application/json',
							},
							body: JSON.stringify(body),
						});

						if (res.ok) {
							mutate();
						}
					} catch (e) {
						console.error(e);
					} finally {
						setUpdating(false);
					}
				},
			)}
		>
			<TextInput {...register('background_color')} label='Background' />
			<TextInput {...register('border_color')} label='Border' />
			<TextInput {...register('text_color')} label='Text' />
			<div style={{ alignSelf: 'center' }}>
				<Tag tag={{ ...tag, ...values }} />
			</div>
			<Button type='submit' loading={updating}>
				Update
			</Button>
		</Stack>
	);
};
