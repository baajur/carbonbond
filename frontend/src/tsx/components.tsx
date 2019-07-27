// 手刻組件庫
import * as React from 'react';
import useOnClickOutside from 'use-onclickoutside';

import '../css/components.css';

type Mode = 'able' | 'warn' | 'disable';

export type Option = {
	name: string,
	mode?: Mode,
	msg?: string
};

const Color = {
	'able': 'inherit',
	'warn': 'red',
	'disable': 'gray'
};

function extractOption(op: string | Option): [string, Mode, string | undefined] {
	if (typeof op == 'string') {
		return [op, 'able', ''];
	} else {
		return [op.name, op.mode || 'able', op.msg];
	}
}

export function DropDown(props: {
	style?: React.CSSProperties,
	selected_style?: React.CSSProperties,
	className?: string,
	option_style?: React.CSSProperties,
	background_style?: React.CSSProperties,
	hover_color?: string,
	value: string,
	options: (Option | string) [],
	onChange: (s: string) => void
}): JSX.Element {
	let [open, setOpen] = React.useState(false);
	let hover_color = props.hover_color || 'inherit';

	let ref = React.useRef(null);
	useOnClickOutside(ref, () => setOpen(false));

	let [main_mode, main_msg] = ((): [Mode, string | undefined] => {
		for (let op of props.options) {
			let [name, mode, msg] = extractOption(op);
			if (name == props.value) {
				return [mode, msg];
			}
		}
		return ['able', undefined]; // 走到這裡代表出問題了
	})();

	return <div ref={ref} style={props.style} styleName='dropDown' className={props.className}>
		<div styleName='Btn' title={main_msg} onClick={() => {
			if (props.options.length > 1) {
				setOpen(!open);
			}
		}}>
			<p style={{ flex: 3 }} />
			<p style={{ flex: 9, textAlign: 'center', color: Color[main_mode] }}>
				{props.value}
			</p>
			<p style={{ flex: 2, textAlign: 'right', transition: '.2s', opacity: open ? 0 : 1 }}>▾</p>
			<p style={{ flex: 1 }} />
		</div>
		<div styleName='Background' style={{
			...props.background_style,
			top: open ? '95%' : '0%',
			opacity: open ? 1 : 0,
			visibility: open ? 'visible' : 'hidden',
		}}>
			{props.options.map((option, i) => {
				let [name, mode, msg] = extractOption(option);
				if (name != props.value) {
					return <div key={i} style={{
						...props.option_style,
						...{ '--hover-color': hover_color } as React.CSSProperties,
						color: Color[mode]
					}} onClick={() => {
						if (mode != 'disable') {
							props.onChange(name);
							setOpen(false);
						}
					}}
					title={msg}
					styleName='Option'>
						<p>{name}</p>
					</div>;
				}
			})}
		</div>
	</div>;
}