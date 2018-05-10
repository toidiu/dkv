import * as React from 'react';

export const App = ({name}) => (
  <div>
    <div>{`Hi ${name}`}</div>
  </div>
);

/* export const Test = ({name}) => ( */
/*   <div> */
/*     <div>{`test ${name}`}</div> */
/*   </div> */
/* ); */

export function Test({name, bla}) {
  return(
    <div>
      <div>{`test ${name}`}</div>
      {`${bla}`}
    </div>
  )
}
