import { Item as ItemType } from '../../types/item';

import { Card, Checkbox, Group, Image, Text } from '@mantine/core';
import * as classes from './Item.css';
import { toLocaleDateString } from '../../utils/date';
import { Tag as TagType } from '../../types/tag';
import { Tag } from '../Tag';
import { useState } from 'react';
import { apiUrl } from '../../api';
import { KeyedMutator } from 'swr';

const SUPPORTED_IMG_EXTENSIONS = ['jpg', 'jpeg', 'webp', 'png'];

export const Item = ({
	item,
	tags,
	auth,
	mutate,
}: {
	tags: Map<string, TagType>;
	item: ItemType & { tags: string[] };
	auth?: string;
	mutate?: KeyedMutator<any>;
}) => {
	const imageUrl = item.image ? URL.parse(item.image) : null;
	const [done, setDone] = useState(item.done);

	if (done) {
		return (
			<Card withBorder padding='xs' radius='md' className={classes.card}>
				<Group justify='center'>
					<Text>Done! 😁👍</Text>
				</Group>
			</Card>
		);
	}

	return (
		<Card withBorder padding='lg' radius='md' className={classes.card}>
			{imageUrl &&
				(imageUrl.pathname.includes('image') ||
					SUPPORTED_IMG_EXTENSIONS.some((ext) =>
						imageUrl.pathname.toLowerCase().endsWith(ext),
					)) && (
					<a href={item.link} target='_blank'>
						<Card.Section mb='sm' className={classes.image}>
							<Image
								src={item.image}
								alt={`${item.title || item.link} preview image`}
								height={180}
							/>
						</Card.Section>
					</a>
				)}

			{item.tags && (
				<Group gap='xs'>
					{item.tags
						.slice(0, 6)
						.map((tagName) => {
							const tag = tags.get(tagName);
							if (tag) {
								return <Tag key={tag.name} tag={tag} />;
							} else {
								return null;
							}
						})}
				</Group>
			)}

			<Text
				component='a'
				href={item.link}
				target='_blank'
				fw={700}
				className={classes.title}
				mt='xs'
			>
				{item.title || item.link.slice(0, 40)}
			</Text>

			<Group justify='space-between' align='flex-end' wrap='nowrap'>
				<div>
					{item.author && <Text fw={500}>{item.author}</Text>}
					{item.published && (
						<Text fz='xs' c='dimmed'>
							posted {toLocaleDateString(item.published)}
						</Text>
					)}
				</div>
				<Checkbox
					size='md'
					radius='xl'
					checked={done}
					disabled={done}
					onChange={async () => {
						if (done) return;
						try {
							const res = await fetch(apiUrl(`/items/${item.id}/done`), {
								method: 'POST',
								headers: {
									['x-auth']: auth ?? '',
								},
							});
							if (res.ok) {
								setDone(true);
								setTimeout(() => mutate?.(), 2000);
							} else {
								setDone(false);
							}
						} catch (e) {
							console.error(e);
							setDone(false);
						}
					}}
				/>

				{/* <ActionIcon
					variant='subtle'
					color='gray'
					onClick={(e) => e.stopPropagation()}
				>
					<TbBookmark size={20} color={theme.colors.yellow[6]} />
				</ActionIcon> */}
			</Group>
		</Card>
	);
};
