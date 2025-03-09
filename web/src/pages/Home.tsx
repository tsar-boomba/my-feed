import useSWR from 'swr';
import { apiFetcher } from '../api';
import { Item as ItemType } from '../types/item';
import {
	Affix,
	Box,
	Center,
	ComboboxData,
	Loader,
	Select,
	Stack,
} from '@mantine/core';
import { useLocalStorage, useViewportSize } from '@mantine/hooks';
import { MOBILE_WIDTH } from '../components/Layout';
import { Item } from '../components/Item';
import { useEffect, useRef } from 'react';
import { TABS_HEIGHT } from '../components/Layout/MobileLayout';
import { useTags } from '../utils/tags';
import { useAuth } from '../utils/useAuth';
import { Masonry } from '../components/Masonry';

const DEFAULT_FROM_LAST = '1w';
const FROM_LAST_OPTIONS: ComboboxData = [
	{
		label: 'Past Day',
		value: '1d',
	},
	{
		label: 'Past 2 Days',
		value: '2d',
	},
	{
		label: 'Past 3 Days',
		value: '3d',
	},
	{
		label: 'Past Week',
		value: '1w',
	},
];

export const Home = () => {
	const [fromLast, setFromLast] = useLocalStorage({
		key: 'fromLast',
		defaultValue: DEFAULT_FROM_LAST,
	});
	const {
		data: items,
		error,
		mutate,
	} = useSWR<(ItemType & { tags: string[] })[]>(
		`/items?from_last=${fromLast}`,
		apiFetcher,
	);
	const [auth] = useAuth();
	const { tags, error: tagsError } = useTags();
	const { width } = useViewportSize();
	const ref = useRef<HTMLDivElement>(null);

	useEffect(() => {
		if (!ref.current) return;
		ref.current.scrollTo({
			top: ref.current.offsetHeight,
		});
	}, [items]);

	if (error) {
		return error.toString();
	}

	if (tagsError) {
		return tagsError.toString();
	}

	if (!items || !tags) {
		return (
			<Center>
				<Loader size='xl' />
			</Center>
		);
	}

	const renderedItems = items.map((item) => (
		<Item tags={tags} item={item} mutate={mutate} auth={auth} key={item.id} />
	));

	if (width <= MOBILE_WIDTH) {
		return (
			<>
				<Stack mx={12} mt={12} gap={12} style={{ overflow: 'auto' }} ref={ref}>
					{renderedItems}
				</Stack>
				<Affix bottom={TABS_HEIGHT + 12} right={12}>
					<Select
						data={FROM_LAST_OPTIONS}
						value={fromLast}
						onChange={(value) => setFromLast(value || DEFAULT_FROM_LAST)}
					/>
				</Affix>
			</>
		);
	}

	return (
		<>
			<Box mx={8} mt={8} pb={36}>
				<Masonry
					items={items}
					columnGutter={16}
					columnWidth={250}
					render={({ data: item }) => (
						<Item tags={tags} item={item} auth={auth} mutate={mutate} />
					)}
				/>
			</Box>
			<Affix bottom={16} right={16}>
				<Select
					data={FROM_LAST_OPTIONS}
					value={fromLast}
					onChange={(value) => setFromLast(value || DEFAULT_FROM_LAST)}
				/>
			</Affix>
		</>
	);
};
