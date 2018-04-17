import React from 'react'
import Router from 'next/router'
import axios from 'axios'

// export default () => <div>Welcome to next.js!</div>

export default class extends React.Component {
  static getInitialProps() {
    return {
      photos: new Array(15).fill(0).map((v, k) => k + 1)
    }
  }

  constructor(props) {
    super(props)

    this.state = {
      posts: []
    }
  }

  componentDidMount() {
    axios
      .get('http://www.reddit.com/r/${this.props.subreddit}.json')
      .then(res => {
        const posts = res.data.data.children.map(obj => obj.data)
        this.setState({ posts })
      })
  }

  render() {
    const { url, photos } = this.props

    return (
      <div className="list">
        <h1>dkv</h1>
        {photos.map(id => (
          <div key={id} className="photo">
            <a
              className="photoLink"
              href={`/photo?id=${id}`}
              onClick={e => this.showPhoto(e, id)}
            >
              {id}
            </a>
          </div>
        ))}
      </div>
    )
  }
}
