/*
 * File: store.tsx
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

import React, { useState, useEffect } from 'react';
import { promisified } from 'tauri/api/tauri';
import { listen } from 'tauri/api/event';

import Context from './context';

const Store: React.FunctionComponent = ({ children }) => {
    // TODO: Add way to specify custom state type
    const [state, setState] = useState<Object | null>(null);

    useEffect(() => {
        // initialize the store
        // TODO: Proper error handling
        promisified({ cmd: 'initializeStore' })
            .then(response => setState(response as Object))
            .catch(err => console.log(err));

        // listen for state updates fired from the backend without a frontend command
        listen('state-update', event => {
            setState(event.payload as Object);
        });
    }, []);

    return <Context.Provider value={{ state, setState }}>
        {children}
    </Context.Provider>;
}

export default Store;
