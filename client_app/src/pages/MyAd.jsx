import React, { Component } from 'react'
import {
  Container, Row, Col,
  Card, CardImg, CardText, CardBody,
  CardTitle, CardSubtitle, Button,
  CardFooter
} from 'reactstrap'
import { Link } from 'react-router-dom'
import NavbarMain from '../partials/NavbarMain'
import NavbarBottom from '../partials/NavbarBottom'
import CssModules from 'react-css-modules'

import dummyImg from '../img/dummy-img.png'
import styles from '../css/MyAd.css'
import ToggleButton from '../partials/ToggleButton'

class MyAd extends Component {
  state = {
    AdStatus: false
  }

  changeAdStatus = () => {
    this.setState({ AdStatus: !this.state.AdStatus })
  }

  renderPartners() {
    return (
      <Container className="basic-container">
        <Row styleName="box">
          <Col md={{ size: 4, offset: 4 }}>
            <Card>
              <CardImg top width="100%" src={dummyImg} alt="Card image cap" />
              <CardBody>
                <CardTitle>Tandry Syawaludin Soedijanto</CardTitle>
                <CardSubtitle styleName="trash-category"><span>Sampah Plastik PVC</span></CardSubtitle>
                <CardText styleName="caption">Jalan Duku 1 Blok C2/25 Pondok Sejahtera, Kelurahan Kutabaru, Kecamatan Pasar Kemis, Kabupaten Tangerang, Banten, 11561</CardText>
                <Button size="sm" block outline>Edit</Button>
              </CardBody>
              <CardFooter styleName={this.state.AdStatus ? 'ad-status active' : 'ad-status'}>
                Ad is {this.state.AdStatus ? "Active" : "Deactive"}
              </CardFooter>
            </Card>
          </Col>
        </Row>

        <Row styleName="toggle-box">
          <Col md={{ size: 4, offset: 4}}>
            <ToggleButton onChange={this.changeAdStatus}/>
          </Col>
        </Row>
      </Container>
    )
  }

  render() {
    return (
      <div styleName="MyAd">
        <NavbarMain prevRoute={this.props.history.goBack} />
        {this.state.showOfferForm ? this.renderOfferForm() : this.renderPartners()}
        <NavbarBottom currentRoute={this.props.location.pathname} />
      </div>
    )
  }
}

export default CssModules(MyAd, styles, { allowMultiple: true })
