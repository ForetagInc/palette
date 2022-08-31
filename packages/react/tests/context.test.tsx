import React, { FC } from 'react';
import { describe, it, expect } from 'bun:test';
import { createPalette, createPaletteContext } from '../source/index';

interface IButtonProps {
	disabled: boolean;
}

describe('React - JSX / TSX', () => {
	const palette = createPalette([
		{
			name: 'light',
			tokens: {
				colors: {
					primary: '#ffffff',
				}
			}
		},
		{
			name: 'dark',
			tokens: {
				colors: {
					primary: '#000000',
				}
			}
		},
	]);

	const { PaletteProvider, usePalette } = createPaletteContext(palette);

	const App = () => {
		return <PaletteProvider theme='light'>
			<p>Test</p>
		</PaletteProvider>
	}

	it('Creates a palette', () => {
		const Button: FC<IButtonProps> = (props) => {
			const { mix, setTheme } = usePalette();

			const classes = mix<IButtonProps>(({ tokens }) => ({
				base: 'bg:white f:blue',
				themes: {
					light: 'true',
				},
				variants: {
					disabled: `bg:red f:${tokens.colors.primary}`,
				}
			}));

			return <a
				className={classes(props)}
				onClick={() => setTheme('dark')}
			>
				Button
			</a>
		}
	});
});