import React, { Component, Fragment } from 'react';
import {
  Container, Row, Col,
  Card, CardImg, CardText, CardBody,
  CardTitle, CardSubtitle, Button,
  ButtonGroup, ButtonDropdown, DropdownToggle,
  DropdownMenu, DropdownItem, CardLink
} from 'reactstrap';
import { Link } from 'react-router-dom';
import NavbarMain from '../partials/NavbarMain';

import '../css/ListOfPartners.css';

class ListOfPartner extends Component {
  constructor(props) {
    super(props)
    this.state = {
      moreAction: false
    }
  }

  toggleMoreAction = () => {
    this.setState({ moreAction: !this.state.moreAction })
  }

  render() {
    return (
      <Fragment>
        <NavbarMain />
        <Container className="home-container">
          <Row>
            <Col md={{ size: 4, offset: 4 }}>
              <Card>
                <CardImg top width="100%" src="https://placeholdit.imgix.net/~text?txtsize=33&txt=318%C3%97180&w=318&h=180" alt="Card image cap" />
                <CardBody>
                  <CardTitle>Tandry Syawaludin Soedijanto</CardTitle>
                  <CardSubtitle className="trash-category"><span>Sampah Plastik PVC</span></CardSubtitle>
                  <CardText>Jalan Duku 1 Blok C2/25 Pondok Sejahtera, Kelurahan Kutabaru, Kecamatan Pasar Kemis, Kabupaten Tangerang, Banten, 11561</CardText>
                  <CardLink href="#">Ajukan Penawaran</CardLink>
                  <CardLink href="#">Another Link</CardLink>
                  {/* <ButtonGroup>
                    <Button>1</Button>
                    <Button>2</Button>
                    <ButtonDropdown isOpen={this.state.moreAction} toggle={this.toggleMoreAction}>
                      <DropdownToggle caret>
                        Dropdown
                      </DropdownToggle>
                      <DropdownMenu>
                        <DropdownItem>Dropdown Link</DropdownItem>
                        <DropdownItem>Dropdown Link</DropdownItem>
                      </DropdownMenu>
                    </ButtonDropdown>
                  </ButtonGroup> */}
                </CardBody>
              </Card>
            </Col>
          </Row>
        </Container>
      </Fragment>
    )
  }
}

export default ListOfPartner