import { Item as ItemType } from '../../types/item';

import { Box, Card, Group, Image, Text } from '@mantine/core';
import * as classes from './Item.css';
import { toLocaleDateString } from '../../utils/date';
import { Tag as TagType } from '../../types/tag';
import { Tag } from '../Tag';

const SUPPORTED_IMG_EXTENSIONS = ['jpg', 'jpeg', 'webp', 'png'];

export const Item = ({
	item,
	tags,
}: {
	tags: Map<string, TagType>;
	item: ItemType & { tags: string[] };
}) => {
	const imageUrl = item.image ? URL.parse(item.image) : null;
	console.log(imageUrl?.pathname);

	return (
		<Card
			withBorder
			padding='lg'
			radius='md'
			className={classes.card}
			component='a'
			href={item.link}
			target='_blank'
		>
			{imageUrl &&
				(imageUrl.pathname.includes('image') ||
					SUPPORTED_IMG_EXTENSIONS.some((ext) =>
						imageUrl.pathname.toLowerCase().endsWith(ext),
					)) && (
					<Card.Section mb='sm' className={classes.image}>
						<Image
							src={item.image}
							alt={`${item.title || item.link} preview image`}
							height={180}
						/>
					</Card.Section>
				)}

			{item.tags && (
				<Group gap='xs'>
					{item.tags
						.slice(0, 20)
						.map((tagName) => {
							const tag = tags.get(tagName);
							if (tag) {
								return <Tag tag={tag} />;
							} else {
								return null;
							}
						})
						.slice(0, 6)}
				</Group>
			)}

			<Text fw={700} className={classes.title} mt='xs'>
				{item.title || item.link.slice(0, 40)}
			</Text>

			<Group wrap='nowrap'>
				<Box>
					{item.author && <Text fw={500}>{item.author}</Text>}
					{item.published && (
						<Text fz='xs' c='dimmed'>
							posted {toLocaleDateString(item.published)}
						</Text>
					)}
				</Box>
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
