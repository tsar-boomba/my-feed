import { MantineSize, Pill } from '@mantine/core';
import { Tag as TagType } from '../types/tag';

export const Tag = ({
	tag,
	size,
	onRemove,
	onClick,
}: {
	tag: TagType;
	size?: MantineSize;
	onRemove?: () => void;
	onClick?: () => void;
}) => {
	return (
		<Pill
			size={size}
			styles={{
				label: {
					height: 'auto',
				},
			}}
			bg={tag.background_color ?? undefined}
			c={tag.text_color ?? undefined}
			style={{
				textTransform: 'uppercase',
				border: tag.border_color ? `1px solid ${tag.border_color}` : undefined,
			}}
			withRemoveButton={!!onRemove}
			onRemove={onRemove}
			onClick={onClick}
			fw={1000}
		>
			{tag.name}
		</Pill>
	);
};
