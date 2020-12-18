/*
 * File: view.js
 * Author: MarkAtk
 * Date: 18.12.20
 *
 * MIT License
 *
 * Copyright (c) 2020 MarkAtk
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

import styled from 'styled-components';

const ViewAxis = styled.div`
    display: flex;
    flex: ${props => props.grow ? '1 1 auto' : 'none'};
    background: ${props => props.background ?? 'none'};
`;

export const ViewRow = styled(ViewAxis)`
    flex-flow: row;
    height: ${props => props.height ?? 'auto'};
`;

export const ViewColumn = styled(ViewAxis)`
    flex-flow: column;
    width: ${props => props.width ?? 'auto'};
`;

export const ViewBody = styled(ViewColumn)`
    height: 100vh;
    flex: 1 1 auto;
`;

export default {
    Row: ViewRow,
    Column: ViewColumn,
    Body: ViewBody
}
