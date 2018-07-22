import React, { Component, Fragment } from 'react';
import {
  Container, Row, Col,
  Card, CardImg, CardText, CardBody,
  CardTitle, CardSubtitle, CardLink,
  Button
} from 'reactstrap';
import { Link } from 'react-router-dom';
import NavbarMain from '../partials/NavbarMain';
import NavbarBottom from '../partials/NavbarBottom';
import TiFlashOutline from 'react-icons/lib/ti/flash-outline'

import dummyImg from '../img/dummy-img.png'
import '../css/MyAdd.css';

class Myadd extends Component {
  constructor(props) {
    super(props)
    this.state = {
    }
  }

  renderPartners() {
    return (
      <Container className="home-container">
        <Row>
          <Col md={{ size: 4, offset: 4 }}>
            <Card>
              <CardImg top width="100%" src={dummyImg} alt="Card image cap" />
              <CardBody>
                <CardTitle>Tandry Syawaludin Soedijanto</CardTitle>
                <CardSubtitle className="trash-category"><span>Sampah Plastik PVC</span></CardSubtitle>
                <CardText>Jalan Duku 1 Blok C2/25 Pondok Sejahtera, Kelurahan Kutabaru, Kecamatan Pasar Kemis, Kabupaten Tangerang, Banten, 11561</CardText>
                <CardLink href="#" onClick={() => this.toggleOfferForm()}><TiFlashOutline /><span>Offer</span></CardLink>
              </CardBody>
            </Card>
          </Col>
        </Row>
      </Container>
    )
  }

  render() {
    return (
      <Fragment>
        <NavbarMain prevRoute={this.props.history.goBack} />
        {this.state.showOfferForm ? this.renderOfferForm() : this.renderPartners()}
        <NavbarBottom currentRoute={this.props.location.pathname} />
      </Fragment>
    )
  }
}

export default Myadd