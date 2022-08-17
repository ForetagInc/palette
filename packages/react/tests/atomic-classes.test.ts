import { describe, it, expect } from 'bun:test';
import { createPalette } from '@/source';

interface IMockButtonProps {
	label: string,
	disabled: boolean,
	uppercase: boolean,
	bold: boolean,
	borderless: boolean,
	rounded: boolean,
	isLoading: boolean,
}

describe('Palette', () => {
	const { mix } = createPalette([
		{
			name: 'screen',
			assets: {
				logo: 'https://foret.ag/logo.png',
			},
			tokens: {
				radius: {
					default: 4
				}
			}
		},
		{
			name: 'xr',
			tokens: {
				radius: {
					default: 8
				}
			}
		}
	]);

	const classes = mix<IMockButtonProps>(({ tokens }) => ({
		base: `r:${tokens.radius.default} b:1|solid|gray-86 f:semibold ~all|100ms|ease p:10|15 f:14|semibold {bg:gray-80}:hover outline:none`,
		variants: {
			uppercase: 't:uppercase',
			disabled: 'cursor:not-allowed',
			isLoading: 'cursor:not-allowed',
			bold: 'f:bold',
			borderless: 'b:none',
			rounded: 'rounded'
		}
	}));

	let generateProps = classes({
		label: 'Button', disabled: true, uppercase: false,
		borderless: false,
		rounded: false,
		bold: false,
		isLoading: false
	});

	it('Generate classes based on conditional variants', async () => {
		expect(generateProps)
			.toBe('r:4 b:1|solid|gray-86 f:semibold ~all|100ms|ease p:10|15 f:14|semibold {bg:gray-80}:hover outline:none cursor:not-allowed');
	});
})