import { mutate } from 'swr';
import { apiUrl } from '../../api';
import { Source } from '../../types/source';
import { useForm } from 'react-hook-form';
import { useState } from 'react';
import { Button, Stack, Text, TextInput } from '@mantine/core';
import { serializeDate } from '../../utils/date';
import { DateInput } from '../form/DateInput';
import { TbPlus, TbX } from 'react-icons/tb';
import { Item } from '../../types/item';
import { closeAllModals } from '@mantine/modals';

type ItemWTags = Item & { tags: string[] };

export const AddSourceModal = ({ auth }: { auth: string }) => {
	const [creatingSource, setCreatingSource] = useState(false);
	const [gettingPreview, setGettingPreview] = useState(false);
	const [preview, setPreview] = useState<ItemWTags[] | null>(null);
	const { register, handleSubmit, getValues, control } = useForm<{
		name: string;
		url: string;
		minDate: Date | null;
	}>({
		defaultValues: {
			name: '',
			url: '',
		},
	});

	const onSubmit = handleSubmit(async ({ name, url, minDate }) => {
		if (!name || !url) return;
		const body: Source = {
			name,
			url,
			minDate: serializeDate(minDate),

			id: 0,
			favorite: false,
			lastPoll: null,
			ttl: null,
			lastPub: new Date().toISOString(),
			createdAt: new Date().toISOString(),
			updatedAt: new Date().toISOString(),
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
				closeAllModals();
				mutate(apiUrl('/sources'));
			} else {
				console.error('Failed to create source:', await res.text());
			}
		} catch (e) {
			console.error(e);
		} finally {
			setCreatingSource(false);
		}
	});

	const getPreview = async () => {
		const { name, url, minDate } = getValues();
		if (!name || !url) return;
		const body: Source = {
			name,
			url,
			minDate: serializeDate(minDate),

			id: 0,
			favorite: false,
			lastPoll: null,
			ttl: null,
			lastPub: new Date().toISOString(),
			createdAt: new Date().toISOString(),
			updatedAt: new Date().toISOString(),
		};

		setGettingPreview(true);
		try {
			const res = await fetch(apiUrl('/sources/preview'), {
				method: 'POST',
				body: JSON.stringify(body, null, 4),
				headers: {
					['x-auth']: auth,
					['content-type']: 'application/json',
				},
			});

			if (!res.ok) return;
			setPreview(await res.json());
		} finally {
			setGettingPreview(false);
		}
	};

	return (
		<Stack component='form' onSubmit={onSubmit}>
			<TextInput {...register('name')} label='Name' withAsterisk />
			<TextInput {...register('url')} label='URL' withAsterisk />
			<DateInput name='minDate' control={control} clearable label='Min Date' />
			{preview && (
				<>
					<Button
						color='red'
						leftSection={<TbX />}
						onClick={() => setPreview(null)}
					>
						Clear
					</Button>
					<Text ta='center' fw={600}>
						Will add {preview.length} items
					</Text>
					{preview.map((p) => (
						<Text>{p.title ?? p.link}</Text>
					))}
				</>
			)}
			<Button
				onClick={getPreview}
				disabled={creatingSource}
				loading={gettingPreview}
			>
				Preview
			</Button>
			<Button
				leftSection={<TbPlus />}
				type='submit'
				disabled={gettingPreview}
				loading={creatingSource}
			>
				Add Source
			</Button>
		</Stack>
	);
};
