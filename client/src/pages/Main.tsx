import * as React from 'react';
import React from 'react'
import { Switch, Route } from 'react-router-dom'

import Dkv from '../components/Dkv.tsx'
import Todo from '../components/Todo.tsx'

const Main = () => (
  <main>
    <Switch>
      <Route exact path='/' component={Dkv}/>
      <Route path='/todo' component={Todo}/>
    </Switch>
  </main>
)

export default Main;
