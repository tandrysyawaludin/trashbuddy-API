import React, { Component, Fragment } from 'react';
import {
  Container, Row, Col,
  Card, CardImg, CardText, CardBody,
  CardTitle, CardSubtitle, CardLink
} from 'reactstrap';
import { Link } from 'react-router-dom';
import NavbarMain from '../partials/NavbarMain';
import NavbarBottom from '../partials/NavbarBottom';
import OfferForm from '../partials/OfferForm';
import TiFlashOutline from 'react-icons/lib/ti/flash-outline'

import '../css/ListOfPartners.css';

class ListOfPartner extends Component {
  constructor(props) {
    super(props)
    this.state = {
      showOfferForm: false
    }
  }

  toggleOfferForm = () => {
    this.setState({ showOfferForm: !this.state.showOfferForm })
  }

  renderOfferForm() {
    return <OfferForm toggleOfferForm={this.toggleOfferForm} />
  }

  renderPartners() {
    return (
      <Container className="home-container">
        {[...Array(10)].map((_, index) =>
          <Row key={index}>
            <Col md={{ size: 4, offset: 4 }}>
              <Card>
                <CardImg top width="100%" src="https://placeholdit.imgix.net/~text?txtsize=33&txt=318%C3%97180&w=318&h=180" alt="Card image cap" />
                <CardBody>
                  <CardTitle>Tandry Syawaludin Soedijanto</CardTitle>
                  <CardSubtitle className="trash-category"><span>Sampah Plastik PVC</span></CardSubtitle>
                  <CardText>Jalan Duku 1 Blok C2/25 Pondok Sejahtera, Kelurahan Kutabaru, Kecamatan Pasar Kemis, Kabupaten Tangerang, Banten, 11561</CardText>
                  <CardLink href="#" onClick={() => this.toggleOfferForm()}><TiFlashOutline /><span>Offer</span></CardLink>
                </CardBody>
              </Card>
            </Col>
          </Row>
        )}
      </Container>
    )
  }

  render() {
    return (
      <Fragment>
        <NavbarMain />
        {this.state.showOfferForm ? this.renderOfferForm() : this.renderPartners()}
        <NavbarBottom />  
      </Fragment>
    )
  }
}

export default ListOfPartner