/*
 * File: connect.tsx
 * Author: MarkAtk
 * Date: 09.12.20
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

import React, { useContext } from 'react';
import { promisified } from 'tauri/api/tauri';

import { Action } from './action';
import Context, { StoreContext } from './context';

export function connect(mapStateToProps: (state: Object) => Object, mapDispatchToProps: (dispatch: (action: Action) => void) => Object) {
    return (Component: typeof React.Component) => {
        return () => {
            const storeState = useContext<StoreContext>(Context);

            // TODO: Add proper error handling
            const dispatch = (action: Action) => {
                promisified(action)
                    .then(response => storeState.setState(response as Object))
                    .catch(err => console.log(err));
            };

            const props = {
                ...mapStateToProps(storeState.state || {}),
                ...mapDispatchToProps(dispatch) ?? {}
            };

            return <Component {...props} />;
        };
    };
}
