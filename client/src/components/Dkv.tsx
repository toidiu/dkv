import * as React from 'react';
/* import proto from '../proto/dkv/dkv_pb.js'; */

class Dkv extends React.Component {
  constructor(props) {
    super(props);
    this.state = {value: ''};

    this.handleChange = this.handleChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  handleChange(event) {
    this.setState({value: event.target.value});
  }

  handleSubmit(event) {
    console.log('A name was submitted: ' + this.state.value);
    /* proto.com.toidiu.dkv.GetKeyRequest.prototype.getKey(1); */

    event.preventDefault();
  }

  render() {
    return (
      <div>
        <div>dkv</div>

        <form onSubmit={this.handleSubmit}>
          <label>
            Name:
            <input type="text" value={this.state.value} onChange={this.handleChange} />
          </label>
          <input type="submit" value="Submit" />
        </form>
      </div>
    );
  }
}

export default Dkv;
