import React, { FC } from 'react';
import { createPalette, createPaletteContext } from '@/source';

const xr = {
	name: 'xr',
	tokens: {
		classes: {
			bg: 'transparent'
		},
		colors: {
			primary: 'F2D2D2'
		}
	}
};

const web = {
	name: 'web',
	tokens: {
		classes: {
			bg: 'white'
		},
		colors: {
			primary: 'F2D2D2'
		}
	}
};

const palette = createPalette([
	xr,
	web
]);

const { PaletteProvider, usePalette } = createPaletteContext(palette);

export const App = () => {
	return <PaletteProvider theme='light'>
		<p>Test</p>
	</PaletteProvider>
}

interface IButtonProps {
	disabled: boolean;
}

export const Button: FC<IButtonProps> = (props) => {
	const { mix, setTheme } = usePalette();

	const classes = mix<IButtonProps>(({ tokens }) => ({
		base: `bg:${tokens.classes.bg}`,
		variants: {
			disabled: `bg:red f:${tokens.colors.primary}`,
		}
	}));

	return <a
		className={classes(props)}
		onClick={() => setTheme('xr')}
	>
		Button
	</a>
}