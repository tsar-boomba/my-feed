import { style } from '@vanilla-extract/css';
import { vars } from '../../theme';

export const card = style({
});

export const image = style({
	borderBottom:
		'1px solid light-dark(var(--mantine-color-gray-2), var(--mantine-color-dark-5))',
});

export const title = style({
	fontFamily: 'Greycliff CF, var(--mantine-font-family)',
	fontSize: vars.fontSizes.lg
});

export const footer = style({
	padding: 'var(--mantine-spacing-xs) var(--mantine-spacing-lg)',
	marginTop: 'var(--mantine-spacing-md)',
	borderTop:
		'1px solid light-dark(var(--mantine-color-gray-2), var(--mantine-color-dark-5))',
});
