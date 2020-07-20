import * as React from 'react';
import { withRouter } from 'react-router-dom';
import { RouteComponentProps } from 'react-router';

import '../../css/bottom_panel.css';
import { EditorPanelState, EditorPanelData } from '../global_state/editor_panel';
import { ajaxOperation, matchErrAndShow } from '../../ts/api';
import { Option, Select } from '../components/select';
import { CategoryBody, checkCanAttach, FieldType } from '../../ts/forum_util';
import { isInteger } from '../../ts/regex_util';
import { EdgeEditor } from './edge_editor';

async function createArticle(data: EditorPanelData | null): Promise<string> {
	if (data) {
		let reply_to = data.edges.map(e => ({ articleId: e.article_id, transfuse: e.transfuse }));
		let res = await ajaxOperation.PostArticle({
			board_name: data.board_name,
			category_name: data.cur_category.name,
			title: data.title,
			content: data.content.slice(0, data.cur_category.structure.length),
			reply_to
		});
		return res.createArticle;
	}
	throw new Error('尚未開始發文');
}

function _EditorPanel(props: RouteComponentProps): JSX.Element | null {
	const { open, editor_panel_data, closeEditorPanel, openEditorPanel, setEditorPanelData }
		= EditorPanelState.useContainer();
	function onTitleClick(): void {
		if (open) {
			closeEditorPanel();
		} else {
			openEditorPanel();
		}
	}
	function deleteEditor(): void {
		// TODO: 跳視窗警告
		let do_delete = true;
		if (editor_panel_data ) {
			if (editor_panel_data.title != '') {
				do_delete = confirm('確定要結束發文？');
			} else {
				for (let c of editor_panel_data.content) {
					if (c != '') {
						do_delete = confirm('確定要結束發文？');
						break;
					}
				}
			}
		}
		if (do_delete) {
			setEditorPanelData(null);
		}
	}
	function onPost(a_id: string): void {
		if (editor_panel_data) {
			props.history.push(`/app/b/${editor_panel_data.board_name}/a/${a_id}`);
			setEditorPanelData(null);
		}
	}
	if (editor_panel_data) {
		return <div styleName="singlePanel editorPanel">
			<div styleName="roomTitle title">
				<div styleName="leftSet">發表文章</div>
				<div onClick={() => onTitleClick()} styleName="middleSet">
					<div style={{ width: '100%', textAlign: 'center' }}>
						b/{editor_panel_data.board_name}
					</div>
				</div>
				<div styleName="rightSet">
					<div styleName="button" onClick={() => deleteEditor()}>✗</div>
				</div>
			</div>
			{
				open ?
					<EditorBody onPost={id => onPost(id)} /> :
					<></>
			}
		</div>;
	} else if (open) {
		// TODO: 錯誤處理，明明沒有文章在編輯，竟然還開著編輯視窗
		return null;
	} else {
		return null;
	}
}

function CategorySelector(): JSX.Element {
	const { editor_panel_data, setEditorPanelData } = EditorPanelState.useContainer();
	if (editor_panel_data) {
		let data = editor_panel_data;
		function onChange(name: string): void {
			for (let c of data.categories) {
				if (c.name == name) {
					setEditorPanelData({ ...data, cur_category: c });
					return;
				}
			}
		}
		let options: Option[] = data.categories.map(c => {
			// 根據鍵結決定哪些分類不可選
			let attach_to = data.edges.map(c => c.category);
			if (checkCanAttach(c, attach_to)) {
				return { name: c.name };
			} else {
				return {
					name: c.name,
					mode: 'warn',
					msg: '鍵結錯誤'
				};
			}
		});
		return <Select
			style={{
				width: 150,
				height: '95%',
				top: '2%',
				zIndex: 1,
				fontSize: 14
			}}
			className="test"
			background_style={{ maxHeight: '60vh' }}
			hover_color="#eee"
			value={data.cur_category.name}
			onChange={s => onChange(s)}
			options={options} />;
	} else {
		throw new Error('尚未開始發文');
	}
}

function SingleFieldInput(props: {
	field: CategoryBody['structure'][0]
	value: string,
	onChange: (s: string) => void,
	onlyOne?: boolean
}): JSX.Element {
	let { field, value, onChange, onlyOne } = props;
	function isValid(): boolean {
		if (field.type == FieldType.Int) {
			return isInteger(value);
		} else if (field.type == FieldType.Rating) {
			throw '尚未實作對 Rating 的檢查';
		} else {
			return true;
		}
	}
	return <>
		{
			onlyOne ?
				null :
				<p styleName="colLabel"> {field.name} ({field.type}) </p>
		}
		{
			(() => {
				switch (field.type) {
					case FieldType.Text:
						return <textarea
							onChange={evt => onChange(evt.target.value)}
							value={value}
							styleName="textInput"
							placeholder={onlyOne ? field.name : ''}
						/>;
					case FieldType.Line:
					case FieldType.Int:
					case FieldType.Rating:
						return <input
							onChange={evt => onChange(evt.target.value)}
							value={value}
							placeholder={onlyOne ? field.name : ''}
							styleName={
								[
									'oneLineInput',
									isValid() ? '' : 'inValid'
								].join(' ')
							}
						/>;
					default:
						return <div>不支援型別：{field.type}</div>;
				}
			})()
		}
	</>;

}

function InputsForStructure(): JSX.Element | null {
	const { setEditorPanelData, editor_panel_data } = EditorPanelState.useContainer();
	if (editor_panel_data) {
		let data = editor_panel_data;
		function onChange(s: string, index: number): void {
			if (editor_panel_data) {
				let ep_data = { ...data };
				ep_data.content[index] = s;
				setEditorPanelData(ep_data);
			}
		}
		let single = data.cur_category.structure.length == 1;
		return <>
			{
				data.cur_category.structure.map((col, i) => (
					<SingleFieldInput key={i} onlyOne={single} field={col}
						value={data.content[i]} onChange={s => onChange(s, i)} />
				))
			}
		</>;
	} else {
		return null;
	}
}

function EditorBody(props: { onPost: (id: string) => void }): JSX.Element {
	const { setEditorPanelData, editor_panel_data } = EditorPanelState.useContainer();
	if (editor_panel_data) {
		let data = editor_panel_data;
		let single = data.cur_category.structure.length == 1;
		let body_style = single ? {} : {
			width: '96%',
			marginLeft: '2%',
			marginRight: '2%',
		};
		return <div styleName="editorBody">
			<div style={{ ...body_style }} styleName="editorInnerBody">
				<div styleName="articleMeta">
					<CategorySelector />
					<input className="articleTitle"
						onChange={evt => {
							setEditorPanelData({
								...data,
								title: evt.target.value
							});
						}}
						value={data.title}
						styleName="oneLineInput"
						placeholder="文章標題"
					/>
				</div>
				{data.edges.length > 0 ? <EdgeEditor/> : null }
				<div styleName="articleContent">
					<InputsForStructure />
					<div>
						<button onClick={() => {
							createArticle(data).then(id => {
								props.onPost(id);
							}).catch(err => {
								matchErrAndShow(err);
							});
						}}>送出文章</button>
						<button>儲存草稿</button>
					</div>
				</div>
			</div>
		</div>;
	} else {
		throw new Error('沒有文章資料卻試圖編輯');
	}
}

const EditorPanel = withRouter(_EditorPanel);
export { EditorPanel };